#include "init.h"
#include "error.h"
#include "vmaware/vmaware.hpp"

bool init() { return VM::detect() == false; }
