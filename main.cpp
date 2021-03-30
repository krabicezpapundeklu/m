#include <iostream>
#include <regex>

#include <boost/asio.hpp>
#include <boost/process.hpp>

#include <Windows.h>

void setup_ansi_console()
{
    auto console_handle = GetStdHandle(STD_OUTPUT_HANDLE);

    if(console_handle == INVALID_HANDLE_VALUE)
    {
        return;
    }

    DWORD console_mode;

    if(!GetConsoleMode(console_handle, &console_mode))
    {
        return;
    }

    if(!SetConsoleMode(console_handle, console_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING))
    {
        return;
    }
}

namespace bp = boost::process;
namespace bs = boost::system;
namespace io = boost::asio;

std::regex progress_regex{R"(Building (.+)\s+\[(\d+)\/(\d+)\])"};

void async_process_output(bp::async_pipe &input_pipe, std::ostream &output_stream, io::streambuf &buffer)
{
    io::async_read_until(input_pipe, buffer, '\n', [&input_pipe, &output_stream, &buffer](const bs::error_code &ec, std::size_t size)
    {
        if(size)
        {
            std::string line;
            std::getline(std::istream{&buffer}, line);

            output_stream << line << '\n';

            std::smatch match;

            if(std::regex_search(line, match, progress_regex))
            {
                std::cout << "\x1b]2;[" << match[2] << '/' << match[3] << "] " << match[1] << '\x07';
            }
        }

        if(!ec)
        {
            async_process_output(input_pipe, output_stream, buffer);
        }
    });
}

int main(int argc, char **argv)
{
    setup_ansi_console();

    std::vector<std::string> args;

    for(auto i = 1; i < argc; ++i)
    {
        args.emplace_back(argv[i]);
    }

    io::io_context io;
    io::signal_set signals{io, SIGINT, SIGTERM};

    bp::group process_group;

    bp::async_pipe stdout_pipe{io};
    bp::async_pipe stderr_pipe{io};

    bp::child mvn
    {
        bp::search_path("mvn"),
        args,
        bp::env["MAVEN_OPTS"] = "-Djansi.passthrough=true",
        bp::std_out > stdout_pipe,
        bp::std_err > stderr_pipe,
        bp::on_exit = [&signals](int exit_code, const std::error_code &ec)
        {
            signals.cancel();
        },
        io,
        process_group
    };

    signals.async_wait([&process_group](const bs::error_code &ec, int signal)
    {
        if(!ec)
        {
            process_group.terminate();
        }
    });

    io::streambuf stdout_buffer;
    io::streambuf stderr_buffer;

    async_process_output(stdout_pipe, std::cout, stdout_buffer);
    async_process_output(stderr_pipe, std::cerr, stderr_buffer);

    io.run();

    return mvn.exit_code();
}
