#include "logger.h"

Logger::Logger() : level(Info) {
}

Logger::~Logger() {
}

void Logger::init(string path) {
	logger_init(path.c_str(), path.length());
}
