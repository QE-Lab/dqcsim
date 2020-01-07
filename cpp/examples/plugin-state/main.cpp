#include <string>

#define DQCSIM_SHORT_LOGGING_MACROS
#include <dqcsim>
using namespace dqcsim::wrap;

ArbData run(std::string *message, RunningPluginState &state, ArbData &&arg) {
  INFO("%s", message->c_str());
  *message = "I was run!";
  return ArbData();
}

int main(int argc, char *argv[]) {
  std::string message = "Hello!";
  int code = Plugin::Frontend("hello", "JvS", "v1.0")
    .with_run(run, &message)
    .run(argc, argv);
  INFO("%s", message.c_str());
  return code;
}
