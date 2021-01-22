[comment]: # (lmake_md_to_doc_comments segment start A)

# new_date_time_units_and_formats

[comment]: # (lmake_cargo_toml_to_md start)

**tutorial for a minimal example of rust wasm PWA**  
***[repo](https://github.com/LucianoBestia/new_date_time_units_and_formats_game); version: 2020.1217.1639  date: 2020-12-19 authors: Luciano Bestia***  

[comment]: # (lmake_cargo_toml_to_md end)

[comment]: # (lmake_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-92-green.svg)](https://github.com/LucianoBestia/new_date_time_units_and_formats/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-7-blue.svg)](https://github.com/LucianoBestia/new_date_time_units_and_formats/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-17-purple.svg)](https://github.com/LucianoBestia/new_date_time_units_and_formats/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/new_date_time_units_and_formats/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/new_date_time_units_and_formats/)

[comment]: # (lmake_lines_of_code end)

[![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/new_date_time_units_and_formats/blob/master/LICENSE) [![Rust](https://github.com/LucianoBestia/new_date_time_units_and_formats/workflows/RustAction/badge.svg)](https://github.com/LucianoBestia/new_date_time_units_and_formats/)

## Try it

<https://bestia.dev/new_date_time_units_and_formats/>  
![screenshot](https://github.com/LucianoBestia/new_date_time_units_and_formats/blob/main/images/screenshot01.jpg?raw=true)

## Proposal for a new date and time units and formats

Let's be innovative !  
I think everybody can agree that the date and time units and formats are an archaic mess.  
It is super difficult to change it in the real world because people learn most of the things in a very early age.  
Later on it is more and more difficult to learn and change.  
And because we live in a multi-generational society this "old" traditions are really tough to change.  
There is always somebody who is unable to learn the new stuff. This is our biology. We are only humans.  
And then there is always some tribal feelings that one group wants to be different from another group. Prehistoric baggage.

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
In the distant past humans were not accustomed to bigger numbers than 60. But today we use bigger numbers in everyday life. At least for money counting. Who doesn't know how to count to 1000 dollars?  
For most practical scenarios it is enough to define time with precisions rounded to 1 minute or 1 milliday.  
This is the formula: millidays = 1000/24\*hours + 1000/(24\*60)\*minutes  
The pronunciation is very similar to what is called "military time" in english.  
People will probably shorten millidays to simply "millis".

| old hour:minute | new milliday | pronunciation                       |
| :-------------- | :----------- | :---------------------------------- |
| 00:00           | 000 md       | zero millidays (midnight)           |
| 06:00           | 250 md       | two fifty millidays                 |
| 12:00           | 500 md       | five hundred millidays (midday)     |
| 18:00           | 750 md       | seven fifty millidays               |
| 23:59           | 999 md       | nine ninety nine millidays          |
| 09:00 - 17:00   | 375 - 708 md | three seventy five to seven hundred | 
| 12:00 - 13:00   | 500 - 542 md | five hundred to five forty two      |
| 20:00 - 22:00   | 833 - 917 md | eight thirty to nine seventeen      |

It is just the same for time intervals. We could say:

| old hour:minute     | new milliday                 |
| :------------------ | :--------------------------- |
| "I work 8 hours"    | "I work 333 millidays"       |
| "Wait 5 minutes"    | "Wait 4 millidays"           |
| "Wait 15 minutes"   | "Wait 10 millidays"          |
| "Drive 1 hour"      | "Drive 42 millidays"         |
| "Movie length 1:30" | "Movie length 62 millidays"  |
| "Cook 2 hours"      | "Cook 83 millidays"          |

Today we are biased with the old units, and to us the new units look strange.
But for a new generation it will be just logical and simple. Just one single unit. Nothing to think a lot about it.

## seconds into decimals of milliday

For very precise measurements today we use seconds and decimals of a second.  
With millidays we can just use decimals of a milliday.  
milliseconds, nanoseconds,... can be converted into microdays, nanodays, picodays,... just follow the SI rules and use what is convenient.  
This is the formula: millidays = 1000/(24\*60\*60)\*seconds
Probably people will shorten microdays to "micros".

| old seconds     | new milliday | new microdays |
| --------------: | -----------: | ------------: |
| 1 s             | 0.012 md     | 12 microdays  |
| 10 s            | 0.116 md     | 116 μd        |
| 10.44 s         | 0.120833 md  | 120.833 μd    |
| 30 s            | 0.347 md     | 347 μd        |
| 60 s            | 0.694 md     | 694 μd        |

The "second" is the base unit of time in the International System of Units (SI). This is unfortunate. It will probably never change.  
The "second" is defined by taking the fixed numerical value of the caesium frequency ∆νCs, the unperturbed ground-state hyperfine transition frequency of the caesium-133 atom, to be 9192631770 when expressed in the unit Hz.  
  
## months and days into fullweeks and weekdays

One year has approximately 365 days that are divided into 12 months.  
Because Earths revolution around the Sun is not exact, we have to add one day every 4 years, except every 100 years, except again every 400 years.  
Confusing? Yes it is.  
The universe doesn't play nice with our human perceptions and simplifications. If you ask an astronomer it is even more complicated than that. We just made some super simple rules. Out of ignorance in the archaic old times. We know better now.  
It is impossible to use the metric system and divide a year into 100 days. We need to follow the natural revolution of our planet. That makes sense.  
Months are a historic mess because humans gained more knowledge over time. From the humble starts when the divine ideal of 360 days was divided into 12 months we came finally to this mess: we have months with 30 and 31 days and one exceptional february with 28 or 29 days. Confusing and not necessary.  
There exists also weeks of 7 days. In history there was an attempt to change it to 10 days, but it failed because of human inability to change basic daily routines and rhythms. So we will sadly leave it to 7 days.  
Every month and every year starts with a different weekday and we need a new calendar every year. That is just comical.  
In the old times months where important for the farmers (95% of the population) to work in the fields. Today the farmers are only 3% of the population. Life and business does not depend on "months" anymore. It is a standard in the big industry to plan, work and communicate in "weeks". And this is smart.  
Let abolish months altogether. There are 52 full weeks in a year, that makes 364 days. Every year, there is 1 day left except every 4 year are 2 days except every 100 are again 1 day and every 400 years 2 days. What to do with this 1 or 2 days at the end of the year?  
CELEBRATION DAYS ! Let celebrate the new year !  
Sometimes for one day and sometimes for 2 days. Most people already celebrates New Years Eve in this fashion.  
This means that every year will start with a monday. Forever. No need for a new calendar. 52 full weeks always the same.   Simple.  
We need to make new ideas to be global, international and computer friendly. Forget about the old cultural differences that caused a lot of problems.  
First we need a new name because week has constantly 7 days. The new word should be "fullweek" abbreviated to "fw".  It starts working on monday and ends sunday to rest after a week of work.  
Quarters for business reports are very easily calculated: 52 fullweeks / 4 = 13 fullweeks per quarter.  
The use of months should disappear in official life.  Sure people will use it in some cultural context in their private life just like we use zodiac signs in astrology today.
Instead of a month, some things would be calculated in intervals of 4 fullweeks. That makes exactly 13 four-weeks intervals in a year. That is very close to the old notion of "month".

| old month day       | new fullweek  | pronunciation                |
| :------------------ | :------------ | :----------------            |
| january 1st         | 01fw 1d       | fullweek one monday          |
| january 2nd         | 01fw 2d       | fullweek one tuesday         |
| january 7th         | 01fw 7d       | fullweek one sunday          |
| january 8th         | 02fw 1d       | fullweek two monday          |
| january 15th        | 03fw 1d       | fullweek three monday        |
| january 22th        | 04fw 1d       | fullweek four monday         |
| january 29th        | 05fw 1d       | fullweek five monday         |
| february 5th        | 06fw 1d       | fullweek six monday          |
| march 5th           | 10fw 1d       | fullweek ten monday          |
| april 2nd           | 14fw 1d       | fullweek fourteen monday     |
| may 7th             | 19fw 1d       | fullweek nineteen monday     |
| june 4th            | 23fw 1d       | fullweek twenty three monday |
| july 2nd            | 27fw 1d       | fullweek twenty seven monday |
| august 6th          | 32fw 1d       | fullweek thirty two monday   |
| september 3rd       | 36fw 1d       | fullweek thirty six monday   |
| october 1st         | 40fw 1d       | fullweek forty monday        |
| november 5th        | 45fw 1d       | fullweek forty five monday   |
| december 3th        | 49fw 1d       | fullweek forty nine monday   |
| december 31th       | 53fw 1d       | celebration day 1            |

## a little rust converter program (pwa, wasm)

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
