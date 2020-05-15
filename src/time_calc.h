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

#ifndef TIME_CALC_H
#define TIME_CALC_H

#include <string>
#include <vector>

class TimeCalc
{
public:
	TimeCalc() = default;
	
	static void calculateDuration(const std::string& timesString);
	
private:	
	struct TimePoint
	{
		TimePoint() = default;
		
		TimePoint(unsigned int hrs, unsigned int mins, unsigned int secs) :
		    hours(hrs), minutes(mins), seconds(secs)
		{			
		}
		
		unsigned int getTotalTimePointInSeconds() const
		{
			unsigned int finalVal = (hours * 60 * 60);
			finalVal += (minutes * 60);
			finalVal += seconds;
			return finalVal;
		}

		unsigned int	hours = 0;
		unsigned int	minutes = 0;
		unsigned int	seconds = 0;
	};
	
	// could use TimePoint instead, but then it would be badly named...
	struct TimePeriod
	{
		TimePeriod() = default;
		
		unsigned int	hours = 0;
		unsigned int	minutes = 0;
		unsigned int	seconds = 0;
		
		void accumulate(const TimePeriod& timePeriod)
		{
			hours += timePeriod.hours;
			minutes += timePeriod.minutes;
			seconds += timePeriod.seconds;
			
			// we also need to normalise
			normaliseUnits();
		}
		
		void addTimePointDeltaInSeconds(unsigned int tpDeltaSecs)
		{
			seconds += tpDeltaSecs;
			
			// we also need to normalise
			normaliseUnits();
		}
		
		void normaliseUnits()
		{
			if (seconds > 59)
			{
				unsigned int numNormalisedMinutes = seconds / 60;
				seconds %= 60;
				
				minutes += numNormalisedMinutes;
			}
			
			if (minutes > 59)
			{
				unsigned int numNormalisedHours = minutes / 60;
				minutes %= 60;
				
				hours += numNormalisedHours;
			}
		}
	};
	
	static TimePoint extractTPFromString(const std::string& timeString);
	
	static TimePeriod calculateTimePeriodFromTPPair(const std::string& tpPair);
	
	// utility helper
	
	static void splitString(const std::string& str, std::vector<std::string>& tokens, const char sep);
};

#endif // TIME_CALC_H
