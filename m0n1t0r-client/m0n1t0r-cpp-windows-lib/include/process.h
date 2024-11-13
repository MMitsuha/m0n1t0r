#pragma once
#include "cxx.h"

struct Output;
struct InteractiveContext;

Output execute(rust::String command, rust::Vec<rust::String> args);
