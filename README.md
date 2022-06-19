[comment]: # (lmake_md_to_doc_comments segment start A)

# new_date_time_units_and_formats

[comment]: # (lmake_cargo_toml_to_md start)

**my proposal for a new date time units and formats**  
***version: 2021.205.1219  date: 2021-02-05 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/new_date_time_units_and_formats)***  

[comment]: # (lmake_cargo_toml_to_md end)

[comment]: # (lmake_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-454-green.svg)](https://github.com/bestia-dev/new_date_time_units_and_formats/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-27-blue.svg)](https://github.com/bestia-dev/new_date_time_units_and_formats/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-18-purple.svg)](https://github.com/bestia-dev/new_date_time_units_and_formats/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/new_date_time_units_and_formats/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/new_date_time_units_and_formats/)

[comment]: # (lmake_lines_of_code end)

[![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/new_date_time_units_and_formats/blob/master/LICENSE)
[![Rust](https://github.com/bestia-dev/new_date_time_units_and_formats/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/new_date_time_units_and_formats/)
[![Hits](https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2Fbestia-dev%2Fnew_date_time_units_and_formats&count_bg=%2379C83D&title_bg=%23555555&icon=&icon_color=%23E7E7E7&title=hits&edge_flat=false)](https://hits.seeyoufarm.com)

Hashtags: #rustlang #pwa #wasm #webassembly #tutorial  
My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## Try it

<https://bestia.dev/new_date_time_units_and_formats/>  
![icon](https://github.com/bestia-dev/new_date_time_units_and_formats/raw/main/web_server_folder/new_date_time_units_and_formats/icons/android-icon-96x96.png)  
![screenshot_01](https://github.com/bestia-dev/new_date_time_units_and_formats/raw/main/images/Screenshot_01s1.png)
![screenshot_02](https://github.com/bestia-dev/new_date_time_units_and_formats/raw/main/images/Screenshot_02s1.png)
![screenshot_03](https://github.com/bestia-dev/new_date_time_units_and_formats/raw/main/images/Screenshot_03s1.png)
![screenshot_04](https://github.com/bestia-dev/new_date_time_units_and_formats/raw/main/images/Screenshot_04s1.png)

## Proposal for a new date and time units and formats

Let's be innovative !  
I think everybody can agree that the date and time units and formats are an archaic mess.  
It is super difficult to change it in the real world because people learn most of the things in a very early age.  
Later on it is more and more difficult to learn and change.  
And because we live in a multi-generational society this "old" traditions are really tough to change.  
There is always somebody who is unable to learn the new stuff. This is our biology. We are only humans.  
And then there is always some tribal feelings that one group wants to be different from another group. Prehistoric baggage.  
But it has already happened many times in history that people changed their units of measurement and formats. So there is a glimpse of hope. I hope without a bloody French revolution and piles of deads, but who knows?  
I know there are already a lot of different proposals out there and I am sure some of them are identical or very similar to mine. Basically with a some thinking, and in the moderne age of knowledge, everybody will get to similar conclusions and solutions.  
<https://www.youtube.com/watch?v=kUIYI34CdkE>  

## hours and minutes into millidays

One day is approximately the time planet Earth needs to revolve around it's axis.  
If you ask an astronomer there is a lot more to that, but for practical use we all agreed how long is one day.  
A day is divided into 24 hours. One hour has 60 minutes and one minute has 60 seconds. This is terrible.  
We all like the metric system. Let's try to change this archaic way of time measurement.  
It is easy and consistent: one day is divided into 1000 millidays.  
The "day" is already an "Acceptable non-SI unit" in the "International System of Units - SI".  
A milliday is similar to a minute. One day has 1440 minutes or 1000 millidays.  
This means that 1 milliday = 1.44 minutes and 1 minute = 0.69444 millidays.  
We could approximate it for easy mental calculations 1 milliday approx. 1.5 minute and 1 minute approx. 0.66 millidays.
We don't really need hours. They are redundant and relics from the past. We could use millidays instead of hours in any scenario: 1 hour = 41.66 millidays.  
In the distant past humans were not accustomed to bigger numbers than 12. You can still here people avoiding big numbers: half past ten AM instead of 10:30, a quarter to eight PM instead of 19:45,...
But today we use bigger numbers in everyday life. At least for money counting. Who doesn't know how to count to 1000 dollars?  
For most practical scenarios it is enough to define time with precisions rounded to 1 minute or 1 milliday.  
This is the formula: millidays = 1000/24\*hours + 1000/(24\*60)\*minutes  
People will probably shorten `millidays` to simply `millis`.

| old hour:minute | new milliday | pronunciation                       |
| :-------------- | :----------- | :---------------------------------- |
| 00:00           | 000 md       | zero millis (midnight)           |
| 06:00           | 250 md       | two fifty millis                 |
| 12:00           | 500 md       | five hundred millis (midday)     |
| 18:00           | 750 md       | seven fifty millis               |
| 23:59           | 999 md       | nine ninety nine millis          |
| 09:00 - 17:00   | 375 - 700 md | three seventy five to seven hundred |
| 12:00 - 13:00   | 500 - 542 md | five hundred to five forty two      |
| 20:00 - 22:00   | 830 - 917 md | eight thirty to nine seventeen      |

It is just the same for time intervals. Today a lot of events are rounded to hours, because this is what we have, but in the future they will be rounded to millidays. I call this a cultural shift to the new unit of measure.  
We could say:

| old hour:minute     | new milliday              | after a cultural shift    |
| :------------------ | :------------------------ | :------------------------ |
| "I work 8 hours"    | "I work 333 millis"       | "I work 300 millis"       |
| "Wait 5 minutes"    | "Wait 4 millis"           | "Wait 5 millis"           |
| "Wait 15 minutes"   | "Wait 10 millis"          | "Wait 10 millis"          |
| "Drive 1 hour"      | "Drive 42 millis"         | "Drive 40 millis"         |
| "Movie length 1:30" | "Movie length 62 millis"  | "Movie length 60 millis"  |
| "Cook 2 hours"      | "Cook 83 millis"          | "Cook 80 millis"          |

Today we are biased with the old units, and to us the new units look strange.
But for a new generation it will be just logical and simple. Just one single unit. Nothing to think a lot about it.

## seconds into microdays

For very precise measurements today we use seconds and decimals of a second.  
With millidays we can just use decimals of a milliday or use smaller units like microdays, nanodays, picodays,... just follow the SI rules and use what is convenient.  
This is the formula: millidays = 1000/(24\*60\*60)\*seconds
Probably people will shorten `microdays` to `micros`.

| old seconds     | new milliday | new microdays |
| --------------: | -----------: | ------------: |
| 1 s             | 0.012 md     | 12 μd         |
| 10 s            | 0.116 md     | 116 μd        |
| 9.58 s          | 0.11088 md   | 110.880 μd    |
| 30 s            | 0.347 md     | 347 μd        |
| 60 s            | 0.694 md     | 694 μd        |
| 86.4 s          | 1 md         | 1000 μd       |

The "second" is the base unit of time in the International System of Units (SI). This is unfortunate. It will probably never change.  
The "second" is defined by taking the fixed numerical value of the caesium frequency ∆νCs, the unperturbed ground-state hyperfine transition frequency of the caesium-133 atom, to be 9192631770 when expressed in the unit Hz.  
From this definition we can see that it is arbitrary and we can define a milliday the same way:  
1 milliday = 9192631770 * 86.4 = 794243384928 ∆νCs

## months and days into veeks and veekdays

One year has approximately 365 days that are divided into 12 months. A group of 7 days is called a week.  
Because Earths revolution around the Sun is not exact, we have to add one day every 4 years, except every 100 years, except again every 400 years.  
Confusing? Yes it is.  
The universe doesn't play nice with our human perceptions and simplifications. If you ask an astronomer it is even more complicated than that. We just made some super simple rules. Out of ignorance in the archaic old times. We know better now.  
It is impossible to use the metric system and divide a year into 100 days. We need to follow the natural revolution of our planet. That makes sense.  

Months are a historic mess because humans gained more knowledge over time. From the humble starts when the divine ideal of 360 days was divided into 12 months we came finally to this mess: we have months with 30 and 31 days and one exceptional february with 28 or 29 days. Confusing and not necessary.  
The date format is so different between cultures that it is sometimes dangerous to misinterpret the numbers: [Date format by country](https://en.wikipedia.org/wiki/Date_format_by_country).  
Every month starts with a different weekday and we need a new calendar every year. That is just comical.  
In the old times months were important for the farmers (95% of the population) to work in the fields. They followed the moon phases, because they didn't have any other way to measure time, but even that is not compatible with the calendar months. Today the farmers are only 3% of the population. Life and business does not depend on "months" anymore. It is standard in the big industry to plan, work and communicate in "weeks". And this is smart.  

The classic week contains 7 days. In history there was an attempt to change it to 10 days, but it failed because of human inability to change basic daily routines and rhythms. It happened during the French revolution. So we will sadly leave it to 7 days. But it is really simple to convert this "new calendar" into a 10-day-week calendar.  

There are 52 full weeks in a year, that makes 364 days. Every year, there is 1 day left except every 4 year are 2 days except every 100 are again 1 day and every 400 years 2 days. What to do with this 1 or 2 days at the end of the year?  

**CELEBRATION DAYS ! Let celebrate the new year !**  

Celebrate for one day or sometimes for 2 days. Most people already celebrate New Years Eve in this fashion.  
This means that every year will start with a monday. Forever. No need for a new calendar. 52 full weeks always the same.  
Simple.  
We need to make new ideas to be global, international and computer friendly. Forget about the old cultural differences that caused a lot of problems. Most of the people don't know there are a lot of different calendars used around the world today and even more  in the past. They just need one calendar and they will be just happy until someone tries to change it.  

First we need a new name for disambiguation because the classic week has constantly 7 days. The new word should be "veek" abbreviated to "v". Same-same, but different. It starts with the working monday and ends sunday to rest after a week of work.  
The veekday is already inside the new date format as a number: 1d is monday and 7d is sunday. So no more unknown veekdays in the date format. And having a numeric value for veekday is way better than words that are different in every languages or cultures today.  

> Estimates of the number of human languages in the world vary between 5000 and 7000.
>
> <https://en.wikipedia.org/wiki/Language>  

I propose that monday, tuesday,... be renamed to 1d, 2d, 3d, 4d, 5d, 6d, 7d with pronunciation one-dee, two-dee, three-dee, four-dee, five-dee, six-dee, seven-dee.  

Quarters for business reports are very easily calculated: 52 veeks / 4 = 13 veeks per quarter.  
Let abolish months altogether. The use of months should disappear in official life.  
Sure people will use it in some cultural context in their private life just like we use zodiac signs in astrology today.
Instead of a month, some things would be calculated in intervals of 4 veeks. That makes exactly 13 four-veeks intervals in a year. That is very close to the old notion of "month", but it is 13 and not 12. A big difference. So, simply don't use it.  

| old month day       | new veek     | pronunciation            |
| :------------------ | :----------- | :----------------        |
| january 1st         | 01v 1d       | veek one one-dee          |
| january 2nd         | 01v 2d       | veek one two-dee          |
| january 7th         | 01v 7d       | veek one seven-dee        |
| january 8th         | 02v 1d       | veek two one-dee          |
| january 15th        | 03v 1d       | veek three one-dee        |
| january 22th        | 04v 1d       | veek four one-dee         |
| january 29th        | 05v 1d       | veek five one-dee         |
| february 5th        | 06v 1d       | veek six one-dee          |
| march 5th           | 10v 1d       | veek ten one-dee          |
| april 2nd           | 14v 1d       | veek fourteen one-dee     |
| may 7th             | 19v 1d       | veek nineteen one-dee     |
| june 4th            | 23v 1d       | veek twenty three one-dee |
| july 2nd            | 27v 1d       | veek twenty seven one-dee |
| august 6th          | 32v 1d       | veek thirty two one-dee   |
| september 3rd       | 36v 1d       | veek thirty six one-dee   |
| october 1st         | 40v 1d       | veek forty one-dee        |
| november 5th        | 45v 1d       | veek forty five one-dee   |
| december 3th        | 49v 1d       | veek forty nine one-dee   |
| december 25th       | 52v 2d       | veek fifty two two-dee    |
| december 31th       | 53v 1d       | celebration one-dee       |

People will probably shorten the pronunciation to `one one-dee`, `forty nine two-dee` without repeating `veek` every single time.  
It is very clear if a number finishes with a unit of measure.  
For years it should be `c` for `CE - common era`, example:  
`2021c`  
to disambiguate from the buddhist year (2564 B.E.), islamic year (1442 AH), Chinese year (4718) and [other years](https://allthatsinteresting.com/what-year-is-it).  
Example of a global full date format with veeks:  
`2021c 23v 3d`  
Now for fun, compare this to one of the many formats today:  
`9.6.2021 tuesday`  
And some more details for the single global format to be very explicit, correct and unambiguous:  

- every number must end with a unit (c, v, d - year CE, veek, day)
- after the unit there is a space  
- veeks are always written with 2 digits example: 01, 02,... because it makes it easier to alphabetically sort dates and never confuse with days  
- days are always one single digit, from 1 to 7
- years are always in 4 digits. The new date format is not suitable for ancient dates before 1000c , but neither is the current calendar. Historians use other types of calendars for the far past.  

## veek calendar forever

I prepared a simple spreadsheet with the new `veek calendar`.  
It is good forever:  
[new_calendar_forever.ods](https://github.com/bestia-dev/new_date_time_units_and_formats/raw/main/web_server_folder/new_date_time_units_and_formats/new_calendar_forever.ods)  
Actually it is so simple, you don't need a calendar, you already know it using simple logic.  
[![calendar 1](https://github.com/bestia-dev/new_date_time_units_and_formats/raw/main/images/calendar_forever_1.png)](https://github.com/bestia-dev/new_date_time_units_and_formats/raw/main/web_server_folder/new_date_time_units_and_formats/new_calendar_forever.ods)  
[![calendar 2](https://github.com/bestia-dev/new_date_time_units_and_formats/raw/main/images/calendar_forever_2.png)](https://github.com/bestia-dev/new_date_time_units_and_formats/raw/main/web_server_folder/new_date_time_units_and_formats/new_calendar_forever.ods)  

## other veeks millis projects

crate:  
<https://github.com/bestia-dev/veeks_millis>  
PWA wasm clock:  
<https://github.com/bestia-dev/veeks_millis_clock>  
![screenshot](https://github.com/bestia-dev/veeks_millis_clock/raw/main/images/compare_clocks.png)  

## Please comment

It is a difficult topic. A lot of people have strong ideas about date and time. Mostly because they were born in a system and learned all the fine details over time. The cultural change to the new system will require a change of a generation. Children that learn the new system early will see the old system as archaic, complicated and not easily understandable.  
Let discuss it in the [Discussion](https://github.com/bestia-dev/new_date_time_units_and_formats/discussions) section of this repository.  Stuff like `time zones`, kiloseconds, swatch internet time,...  
But most of all let strive for change for the better. All of humanity together.  

## TL;DR - The shortest description

There must be a lot of compromises to adapt our time units to our planet's orbit.  
A new standard should explicitly state chosen compromises for example (my proposal):  
 
- Make hours, minutes, and seconds obsolete. The only unit for time should be the already existing "day". Smaller is a milliday or microday. Logical. Metric. The "day" is defined as being equal to the time duration of 794 243 384 928 000 periods of the radiation corresponding to the transition between the two hyperfine levels of the fundamental unperturbed ground-state of the caesium-133 atom.
- An Earth's natural day is close to 1d=1000md, but not equal. Sometimes there is a leap second or microday to synchronize them. 
- Time zones are very natural to people. I think they must stay. Today they are mostly rounded to one hour, but that can change arbitrarily. We have UTC to express a global planet time.

example: 6:00 AM = 250md, 12:00 AM = 500md, 6:00pm = 750md  
  
- Make months obsolete, they are not needed if we have weeks.
- A week has 7 days only because people have this tradition and built a lot of routines around that. This is impossible to change.
- Let start every year with a "Monday".  So we know for every day in the year which weekday it is. Forever.
- A year is not exactly 52 weeks. The remaining 1 or 2 "celebration days" are added to sync our measurement to the planet's orbit.
- So we need to create a new term for this new type of week that has the exception of "celebration days". Let call it "veek" for fun.
- The weekday's names are different in every language. Let make things global and digital. monday is 1d, tuesday is 2d,.., sunday is 7d.

example: 1.1.2021 = 2021c 01v 1d,  11.3.2021 = 2021c 10v 7d  
"c" stands for CE-common era. "v" stands for veek, "d" stands for day.  

## Development of a little rust converter program (pwa, wasm) for mobile

Install some utilities:  
`cargo install cargo-make`  
`cargo install lmake_version_from_date`  
`cargo install basic-http-server`  
Run  
`cargo make` - for help  
Run  
`cargo make release` - build the wasm pkg  

Run the web server in a separate terminal so it can stay running all the time.  
Go to the web server folder and run the server:  
`cd ~/rustprojects/new_date_time_units_and_formats/web_server_folder; basic-http-server`  
Open the browser on:  
<http://127.0.0.1:4000/new_date_time_units_and_formats/>  
  
This is a PWA application and works offline inside the browser with wasm.  
It is made for mobile browsers. In the desktop browser press F12 and choose a  mobile format.  
