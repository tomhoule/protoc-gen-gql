const PizzaSauce = `
# Choose the right one
enum PizzaSauce {
  TOMATO
  # Congratulations, you have good taste
  CREAM
  WITHOUT
  JAM
  BARBECUE
}
`

const ToppingFatContentFatType = `
# TODO: see with a doctor
enum ToppingFatContentFatType {
  UNKNOWN
  # Like avocado
  GOOD
  BAD
}
`

const TopLevelEnum = `
# A top level enum
enum TopLevelEnum {
  UNKNOWN
  TOP
  LEVEL
}
`

const GoogleProtobufTimestamp = `
# A Timestamp represents a point in time independent of any time zone
# or calendar, represented as seconds and fractions of seconds at
# nanosecond resolution in UTC Epoch time. It is encoded using the
# Proleptic Gregorian Calendar which extends the Gregorian calendar
# backwards to year one. It is encoded assuming all minutes are 60
# seconds long, i.e. leap seconds are "smeared" so that no leap second
# table is needed for interpretation. Range is from
# 0001-01-01T00:00:00Z to 9999-12-31T23:59:59.999999999Z.
# By restricting to that range, we ensure that we can convert to
# and from  RFC 3339 date strings.
# See [https://www.ietf.org/rfc/rfc3339.txt](https://www.ietf.org/rfc/rfc3339.txt).
#
# # Examples
#
# Example 1: Compute Timestamp from POSIX \`time()\`.
#
#     Timestamp timestamp;
#     timestamp.set_seconds(time(NULL));
#     timestamp.set_nanos(0);
#
# Example 2: Compute Timestamp from POSIX \`gettimeofday()\`.
#
#     struct timeval tv;
#     gettimeofday(&tv, NULL);
#
#     Timestamp timestamp;
#     timestamp.set_seconds(tv.tv_sec);
#     timestamp.set_nanos(tv.tv_usec * 1000);
#
# Example 3: Compute Timestamp from Win32 \`GetSystemTimeAsFileTime()\`.
#
#     FILETIME ft;
#     GetSystemTimeAsFileTime(&ft);
#     UINT64 ticks = (((UINT64)ft.dwHighDateTime) << 32) | ft.dwLowDateTime;
#
#     // A Windows tick is 100 nanoseconds. Windows epoch 1601-01-01T00:00:00Z
#     // is 11644473600 seconds before Unix epoch 1970-01-01T00:00:00Z.
#     Timestamp timestamp;
#     timestamp.set_seconds((INT64) ((ticks / 10000000) - 11644473600LL));
#     timestamp.set_nanos((INT32) ((ticks % 10000000) * 100));
#
# Example 4: Compute Timestamp from Java \`System.currentTimeMillis()\`.
#
#     long millis = System.currentTimeMillis();
#
#     Timestamp timestamp = Timestamp.newBuilder().setSeconds(millis / 1000)
#         .setNanos((int) ((millis % 1000) * 1000000)).build();
#
#
# Example 5: Compute Timestamp from current time in Python.
#
#     timestamp = Timestamp()
#     timestamp.GetCurrentTime()
#
# # JSON Mapping
#
# In JSON format, the Timestamp type is encoded as a string in the
# [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format. That is, the
# format is "{year}-{month}-{day}T{hour}:{min}:{sec}[.{frac_sec}]Z"
# where {year} is always expressed using four digits while {month}, {day},
# {hour}, {min}, and {sec} are zero-padded to two digits each. The fractional
# seconds, which can go up to 9 digits (i.e. up to 1 nanosecond resolution),
# are optional. The "Z" suffix indicates the timezone ("UTC"); the timezone
# is required, though only UTC (as indicated by "Z") is presently supported.
#
# For example, "2017-01-15T01:30:15.01Z" encodes 15.01 seconds past
# 01:30 UTC on January 15, 2017.
#
# In JavaScript, one can convert a Date object to this format using the
# standard [toISOString()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString]
# method. In Python, a standard \`datetime.datetime\` object can be converted
# to this format using [\`strftime\`](https://docs.python.org/2/library/time.html#time.strftime)
# with the time format spec '%Y-%m-%dT%H:%M:%S.%fZ'. Likewise, in Java, one
# can use the Joda Time's [\`ISODateTimeFormat.dateTime()\`](
# http://www.joda.org/joda-time/apidocs/org/joda/time/format/ISODateTimeFormat.html#dateTime--)
# to obtain a formatter capable of generating timestamps in this format.
#
#
type GoogleProtobufTimestamp {
  # Represents seconds of UTC time since Unix epoch
  # 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
  # 9999-12-31T23:59:59Z inclusive.
  seconds: Int!
  # Non-negative fractions of a second at nanosecond resolution. Negative
  # second values with fractions must still have non-negative nanos values
  # that count forward in time. Must be from 0 to 999,999,999
  # inclusive.
  nanos: Int!
}
`

const GoogleProtobufTimestampInput = `
# A Timestamp represents a point in time independent of any time zone
# or calendar, represented as seconds and fractions of seconds at
# nanosecond resolution in UTC Epoch time. It is encoded using the
# Proleptic Gregorian Calendar which extends the Gregorian calendar
# backwards to year one. It is encoded assuming all minutes are 60
# seconds long, i.e. leap seconds are "smeared" so that no leap second
# table is needed for interpretation. Range is from
# 0001-01-01T00:00:00Z to 9999-12-31T23:59:59.999999999Z.
# By restricting to that range, we ensure that we can convert to
# and from  RFC 3339 date strings.
# See [https://www.ietf.org/rfc/rfc3339.txt](https://www.ietf.org/rfc/rfc3339.txt).
#
# # Examples
#
# Example 1: Compute Timestamp from POSIX \`time()\`.
#
#     Timestamp timestamp;
#     timestamp.set_seconds(time(NULL));
#     timestamp.set_nanos(0);
#
# Example 2: Compute Timestamp from POSIX \`gettimeofday()\`.
#
#     struct timeval tv;
#     gettimeofday(&tv, NULL);
#
#     Timestamp timestamp;
#     timestamp.set_seconds(tv.tv_sec);
#     timestamp.set_nanos(tv.tv_usec * 1000);
#
# Example 3: Compute Timestamp from Win32 \`GetSystemTimeAsFileTime()\`.
#
#     FILETIME ft;
#     GetSystemTimeAsFileTime(&ft);
#     UINT64 ticks = (((UINT64)ft.dwHighDateTime) << 32) | ft.dwLowDateTime;
#
#     // A Windows tick is 100 nanoseconds. Windows epoch 1601-01-01T00:00:00Z
#     // is 11644473600 seconds before Unix epoch 1970-01-01T00:00:00Z.
#     Timestamp timestamp;
#     timestamp.set_seconds((INT64) ((ticks / 10000000) - 11644473600LL));
#     timestamp.set_nanos((INT32) ((ticks % 10000000) * 100));
#
# Example 4: Compute Timestamp from Java \`System.currentTimeMillis()\`.
#
#     long millis = System.currentTimeMillis();
#
#     Timestamp timestamp = Timestamp.newBuilder().setSeconds(millis / 1000)
#         .setNanos((int) ((millis % 1000) * 1000000)).build();
#
#
# Example 5: Compute Timestamp from current time in Python.
#
#     timestamp = Timestamp()
#     timestamp.GetCurrentTime()
#
# # JSON Mapping
#
# In JSON format, the Timestamp type is encoded as a string in the
# [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format. That is, the
# format is "{year}-{month}-{day}T{hour}:{min}:{sec}[.{frac_sec}]Z"
# where {year} is always expressed using four digits while {month}, {day},
# {hour}, {min}, and {sec} are zero-padded to two digits each. The fractional
# seconds, which can go up to 9 digits (i.e. up to 1 nanosecond resolution),
# are optional. The "Z" suffix indicates the timezone ("UTC"); the timezone
# is required, though only UTC (as indicated by "Z") is presently supported.
#
# For example, "2017-01-15T01:30:15.01Z" encodes 15.01 seconds past
# 01:30 UTC on January 15, 2017.
#
# In JavaScript, one can convert a Date object to this format using the
# standard [toISOString()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString]
# method. In Python, a standard \`datetime.datetime\` object can be converted
# to this format using [\`strftime\`](https://docs.python.org/2/library/time.html#time.strftime)
# with the time format spec '%Y-%m-%dT%H:%M:%S.%fZ'. Likewise, in Java, one
# can use the Joda Time's [\`ISODateTimeFormat.dateTime()\`](
# http://www.joda.org/joda-time/apidocs/org/joda/time/format/ISODateTimeFormat.html#dateTime--)
# to obtain a formatter capable of generating timestamps in this format.
#
#
input GoogleProtobufTimestampInput {
  # Represents seconds of UTC time since Unix epoch
  # 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
  # 9999-12-31T23:59:59Z inclusive.
  seconds: Int
  # Non-negative fractions of a second at nanosecond resolution. Negative
  # second values with fractions must still have non-negative nanos values
  # that count forward in time. Must be from 0 to 999,999,999
  # inclusive.
  nanos: Int
}
`

const Pizza = `
# A pizza, with toppings and stuff
type Pizza {
  # How the pizza is usually called
  title: String!
  # Where the pizza is from
  origin: String!
  baked_at: GoogleProtobufTimestamp!
  # What toppings the pizza has
  toppings: [Topping]!
  # Right or wrong sauce
  sauce: PizzaSauce!
}
`

const PizzaInput = `
# A pizza, with toppings and stuff
input PizzaInput {
  # How the pizza is usually called
  title: String
  # Where the pizza is from
  origin: String
  baked_at: GoogleProtobufTimestampInput
  # What toppings the pizza has
  toppings: [ToppingInput]
  # Right or wrong sauce
  sauce: PizzaSauce
}
`

const Topping = `
# Describes a Pizza topping
type Topping {
  name: String!
  # Is it sweet?
  # ...or what
  sweet: Boolean!
  vitamins: [String]!
}
`

const ToppingInput = `
# Describes a Pizza topping
input ToppingInput {
  name: String
  # Is it sweet?
  # ...or what
  sweet: Boolean
  vitamins: [String]
}
`

const ToppingFatContent = `
# What's the fat content of this topping, for people who care
type ToppingFatContent {
  # How much, out of 100
  percentage: Int!
  type: ToppingFatContentFatType!
}
`

const ToppingFatContentInput = `
# What's the fat content of this topping, for people who care
input ToppingFatContentInput {
  # How much, out of 100
  percentage: Int
  type: ToppingFatContentFatType
}
`

const Pizzeria = `
type PizzeriaService {
  makeSimplePizza(topping: ToppingInput!): Pizza!
  observePizzas(topping: ToppingInput!): Pizza!
}
`

const Query = `
type Query {
  pizzeria: PizzeriaService!
}
`

module.exports = [
  PizzaSauce,
  ToppingFatContentFatType,
  TopLevelEnum,
  GoogleProtobufTimestamp,
  GoogleProtobufTimestampInput,
  Pizza,
  PizzaInput,
  Topping,
  ToppingInput,
  ToppingFatContent,
  ToppingFatContentInput,
  Pizzeria,
  Query,
]
