/*
 Timecalc
 Copyright 2020 Peter Pearson.

 Licensed under the Apache License, Version 2.0 (the "License");
 You may not use this file except in compliance with the License.
 You may obtain a copy of the License at

 http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
 ---------
*/

#include <stdio.h>
#include <cstring>

#include "time_calc.h"

int main(int argc, char** argv)
{
	if (argc < 2)
	{
		fprintf(stderr, "Error: timecalc requires a command line argument consisting of at least one pair of time strings, i.e. '08:25-14:50'\n");
		return 0;
	}
	
	if (strstr(argv[1], "-version"))
	{
		fprintf(stdout, "Timecalc 1.0 (C++ version).\n");
		return 0;
	}
	
	std::string timeValues(argv[1]);	
	TimeCalc::calculateDuration(timeValues);
	
	return 0;
}

