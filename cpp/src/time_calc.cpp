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

#include "time_calc.h"

#include <cstdlib>
#include <ctime>

void TimeCalc::calculateDuration(const std::string& timesString)
{
	TimePeriod totalTimePeriod;
	
	// see if we have more than one pairs of times
	if (timesString.find(',') != std::string::npos)
	{
		// we have multiple time pairs
		std::vector<std::string> stringPairs;
		splitString(timesString, stringPairs, ',');
		
		for (const std::string& timePairStr : stringPairs)
		{
			TimePeriod pairTimePeriod = calculateTimePeriodFromTPPair(timePairStr);
			
			if (pairTimePeriod.isNull())
			{
				fprintf(stderr, "Error calculating time period from supplied input value.\n");
				return;
			}
			
			totalTimePeriod.accumulate(pairTimePeriod);
		}
	}
	else
	{
		// we only have one, so...
		
		totalTimePeriod = calculateTimePeriodFromTPPair(timesString);
		
		if (totalTimePeriod.isNull())
		{
			fprintf(stderr, "Error calculating time period from supplied input value.\n");
			return;
		}
	}
	
	if (totalTimePeriod.hours > 0 && totalTimePeriod.seconds > 0)
	{
		fprintf(stdout, "Total time: %u %s, %u %s, %u %s.\n",
				totalTimePeriod.hours, (totalTimePeriod.hours == 1) ? "hour" : "hours",
				totalTimePeriod.minutes, (totalTimePeriod.minutes == 1) ? "minute" : "minutes",
				totalTimePeriod.seconds, (totalTimePeriod.seconds == 1) ? "second" : "seconds");
	}
	else if (totalTimePeriod.hours > 0)
	{
		// no seconds to worry about.
		fprintf(stdout, "Total time: %u %s, %u %s.\n",
				totalTimePeriod.hours, (totalTimePeriod.hours == 1) ? "hour" : "hours",
				totalTimePeriod.minutes, (totalTimePeriod.minutes == 1) ? "minute" : "minutes");
	}
	else if (totalTimePeriod.seconds > 0)
	{
		// just minutes and seconds
		fprintf(stdout, "Total time: %u %s, %u %s.\n",
				totalTimePeriod.minutes, (totalTimePeriod.minutes == 1) ? "minute" : "minutes",
				totalTimePeriod.seconds, (totalTimePeriod.seconds == 1) ? "second" : "seconds");
	}
	else
	{
		// hopefully just minutes
		fprintf(stdout, "Total time: %u %s.\n",
				totalTimePeriod.minutes, (totalTimePeriod.minutes == 1) ? "minute" : "minutes");
	}
}

TimeCalc::TimePoint TimeCalc::extractTPFromString(const std::string& timeString)
{
	TimePoint tp;
	
	unsigned int numColons = 0;
	for (const char chr : timeString)
	{
		if (chr == ':')
			numColons ++;
	}
	
	if (numColons == 0)
	{
		// see if string is "now"
		if (timeString == "now")
		{
			time_t currentTime = time(0);
			struct tm* timeS = localtime(&currentTime);
			
			tp.hours = timeS->tm_hour;
			tp.minutes = timeS->tm_min;
			// don't bother with seconds for the moment...
			
			// TODO: attempt to work out the precision of all over string values provided (outside of this
			//       function), to see if we need seconds or not.
		}		
		
		return tp;
	}
	else if (numColons == 1)
	{
		// only hours and minutes
		if (sscanf(timeString.c_str(), "%u:%u", &tp.hours, &tp.minutes) != 2)
		{
			// something was wrong, so reset values
			tp.reset();
		}
	}
	else if (numColons == 2)
	{
		// hours, minutes and seconds
		if (sscanf(timeString.c_str(), "%u:%u:%u", &tp.hours, &tp.minutes, &tp.seconds) != 3)
		{
			// something was wrong, so reset values
			tp.reset();
		}
	}
	
	return tp;
}

TimeCalc::TimePeriod TimeCalc::calculateTimePeriodFromTPPair(const std::string& tpPair)
{
	TimePeriod timePeriod;
	
	size_t sepPos = tpPair.find('-');
	if (sepPos != std::string::npos)
	{
		std::string tpStartStr = tpPair.substr(0, sepPos);		
		std::string tpEndStr = tpPair.substr(sepPos + 1);
		
		TimePoint startTime = extractTPFromString(tpStartStr);
		TimePoint endTime = extractTPFromString(tpEndStr);
		
		if (startTime.isNull())
		{
			fprintf(stderr, "Error: unrecognised value in '%s'.\n", tpStartStr.c_str());
			return timePeriod;
		}
		if (endTime.isNull())
		{
			fprintf(stderr, "Error: unrecognised value in '%s'.\n", tpEndStr.c_str());
			return timePeriod;
		}
		
		unsigned int tpDeltaSeconds = endTime.getTotalTimePointInSeconds() - startTime.getTotalTimePointInSeconds();
		timePeriod.addTimePointDeltaInSeconds(tpDeltaSeconds);
	}
	
	return timePeriod;
}

void TimeCalc::splitString(const std::string& str, std::vector<std::string>& tokens, const char sep)
{
	size_t lastPos = str.find_first_not_of(sep, 0);
	size_t pos = str.find_first_of(sep, lastPos);

	while (lastPos != std::string::npos || pos != std::string::npos)
	{
		tokens.push_back(str.substr(lastPos, pos - lastPos));
		lastPos = str.find_first_not_of(sep, pos);
		pos = str.find_first_of(sep, lastPos);
	}
}
