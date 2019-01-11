#include <iostream>
#include <cstring>
#include <string>

using namespace std;

#ifdef __cplusplus
extern "C"{
#endif
		void logger_init(const char* c, unsigned int len);
		void debug(const char* c, unsigned int len);
#ifdef __cplusplus
}
#endif

enum LogLevel {
    Debug = 1,
    Info  = 2,
    Warn  = 4,
    Error = 8,
    Fatal = 16,
};

class Logger {
public:
	Logger();
	~Logger();
	static void init(string path);
	void end() {
		switch (level) {
			case Debug:{
				debug(log_content.c_str(), log_content.length());
			    break;
			}
			case Info :{
				break;
			}
			case Warn :{
			    break;
			}
			case Error:{
			    break;
			}
			case Fatal:{
			    break;
			}
			default: 
				break;
		}
	}
private:
	LogLevel level;
	string log_content;
};
