//! # Timetableau: a timetable management application for Highfield.
//!
//! Timetableau is a timetable management application for [the Highfield school]
//! and [the Fearnhill school] allowing for the creation, management, and
//! storage of timetables for both students and teachers at either school.
//!
//! It should be noted that the application is intended for students
//! at the Highfield school, not for Fearnhill students: the only reason
//! Fearnhill support is provided is due to the consortium Highfield is in
//! with Fearnhill -- if Fearnhill support was not provided, Highfield sixth
//! form students (who may utilise the consortium) would be unable to use the
//! application.
//!
//! ## Room Numbering Scheme
//!
//! A room numbering scheme is the scheme used by buildings to assign their
//! rooms unique identifiers (allowing persons within that building to easily
//! find rooms to which they've never been or for persons to easily refer to
//! rooms within that building).
//!
//! Ideally, room numbering schemes should include context about that room's
//! location (and, possibly, purpose) to further increase readability -- for
//! example:
//!
//! * `room-22` is a very poor room number as a majority of it is occupied
//!   by "boilerplate" (i.e., `room-` does nothing the help identify or
//!   describe the room, as each room will have that same prefix) and `22`
//!   contains no information as to the room's location.
//!
//! * `F5R22` is much more better: it isn't occupied by a redundant prefix and
//!   it describes the floor on which the room is located (increasing its
//!   locatability).
//!
//! ### Highfield's Room Numbering Scheme
//!
//! Highfield has the following room numbering scheme: `BF##`:
//!
//! * The `B` refers to the block in which the room is located -- this can
//!   be any one of the following: `H` for Howard, `P` for Parker, and `U`
//!   for Unwin.
//!
//! * The `F` refers to the floor on which the room is located -- this can
//!   be `G` for the ground floor, or `n` for the nth floor (note: `n` must
//!   be a single digit number).
//!
//! * The `##` refers to the discriminator -- there are multiple rooms on
//!   every floor of every block: the discriminator, which is a two digit
//!   number, allows persons to distinguish between these rooms.
//!
//! Here are some examples of Highfield rooms:
//!
//! * `HG01` refers to a room on the ground floor of the Howard block with
//!   discriminator `01`.
//!
//! * `P212` refers to a room on the second floor of the Parker block with
//!   discriminator `12`.
//!
//! * `U111` refers to a room on the first floor of the Unwin block with
//!   discriminator `11`.
//!
//! ### Fearnhill's Room Numbering Scheme
//!
//! At the time of writing, Fearnhill's room numbering scheme remains unknown.
//!
//! ## The timetable format
//!
//! ### What is a timetable
//!
//! A timetable is a document (or data structure) which describes a set of
//! activities a person has to undertake on a recurring basis (typically used
//! to describe the lessons/courses of a student/teacher) -- each full
//! completion of the timetable is referred to as an iteration of that
//! timetable. For example, a university professor might have a weekly timetable
//! which describes the lessons they have every week (i.e., the timetable
//! contains one week of lessons, which is repeated every week).
//!
//! It should be noted that it is possible to deviate from the timetable --
//! irregular/uncommon deviations from the timetable should have no effect
//! on that timetable's perceived suitability. For example, if a student
//! contracts an illness (which prevents them from attending school), they
//! may temporarily deviate from that timetable (and stay at home) until
//! their condition improves.
//!
//! ### Highfield's Timetable
//!
//! Both Highfield and Fearnhill use a two-week alternating timetable -- this
//! means that the timetables given to students and teachers contain lesson
//! information about two separate weeks (referred to as week one -- the first
//! week -- and week two -- the second) which start on a Monday and end on a
//! Sunday, where each week alternates after the other one finishes (for
//! example, a week two Sunday is followed by a week one Monday and vice
//! versa).
//!
//! Each week contains information about a set of `5` days (Monday to Friday)
//! on which activities can occur (both schools close on Saturdays and Sundays
//! to allow both the students and the teachers to rest/relax without affecting
//! the education other students receive -- as a result, it is impossible
//! for activities to occur on a Saturday or a Sunday). Days on which
//! activities can occur will henceforth be referred to as "active days" and,
//! consequently, days on which activities cannot occur will be referred to as
//! "inactive days".
//!
//! Each active day, in turn, contains information about a set of `5` periods
//! during which an activity can occur. It is impossible for a lesson to
//! occur outside of a pre-approved period, or for a period to only
//! be partially utilised (i.e., it is impossible to use only a fraction of
//! a period) -- it should be noted that it is perfectly possible for an
//! activity to span multiple periods.
//!
//! The pre-approved periods are as follows (these periods are the same
//! for every active day):
//!
//! | Period    | Start Time | End Time |
//! |-----------|------------|----------|
//! | First     | 08:50      | 09:50    |
//! | Second    | 09:50      | 10:50    |
//! | Third     | 11:10      | 12:10    |
//! | Fourth    | 12:10      | 13:10    |
//! | Fifth     | 13:55      | 14:55    |
//!
//! It should be noted that the end time is **not** included in the period
//! (e.g., `09:50` belongs to the second time slot, not the first).
//!
//! ### Example Timetable
//!
//! <div align="center">
//!
//! **WEEK ONE**
//!
//! |       |  Monday | Tuesday | Wednesday |  Thursday |   Friday  |
//! |:-----:|:-------:|:-------:|:---------:|:---------:|:---------:|
//! | 08:25 |  TUTOR  |  TUTOR  |   TUTOR   |   TUTOR   |   TUTOR   |
//! | 08:50 | English |  Maths  |  English  |   Maths   |   French  |
//! | 09:50 | History |   R.S.  |    P.E.   |  English  |  History  |
//! | 10:50 |  BREAK  |  BREAK  |   BREAK   |   BREAK   |   BREAK   |
//! | 11:10 |  Maths  |   R.M.  |    R.S.   |  History  |  Physics  |
//! | 12:10 | Physics | Physics |   Maths   |   French  |  Biology  |
//! | 13:10 |  LUNCH  |  LUNCH  |   LUNCH   |   LUNCH   |   LUNCH   |
//! | 13:55 |   P.E.  | Biology |  Biology  | Chemistry | Chemistry |
//!
//! <br>
//!
//! **WEEK TWO**
//!
//! |       |  Monday | Tuesday | Wednesday |  Thursday |   Friday  |
//! |:-----:|:-------:|:-------:|:---------:|:---------:|:---------:|
//! | 08:25 |  TUTOR  |  TUTOR  |   TUTOR   |   TUTOR   |   TUTOR   |
//! | 08:50 | English |  Maths  |  English  |   Maths   |   French  |
//! | 09:50 | History |   R.M.  |    P.E.   |  English  |  History  |
//! | 10:50 |  BREAK  |  BREAK  |   BREAK   |   BREAK   |   BREAK   |
//! | 11:10 |  Maths  |   R.S.  |    R.S.   |  History  |  Physics  |
//! | 12:10 | Physics | Physics |   Maths   |   French  |  Biology  |
//! | 13:10 |  LUNCH  |  LUNCH  |   LUNCH   |   LUNCH   |   LUNCH   |
//! | 13:55 |   P.E.  | Biology |  Biology  | Chemistry | Chemistry |
//!
//! </div>
//!
//! ### WDF Notation
//!
//! WDF notation is a shorthand method of referring to an individual
//! timeslot within Highfield's timetable -- it takes the form of `W#D#P#`:
//!
//! * The first `#` refers to the week (this can be either `1` or `2`).
//!
//! * The second `#` refers to the day (this is the first letter of the
//!    weekday of the period -- `R` is used to represent Thursday, Saturday
//!    and Sunday are inactive days and cannot be referred to using this
//!    notation).
//!
//! * The third and final `#` refers to the period (this has to be a number
//!    between `1`, for first, and `5`, for fifth, inclusive).
//!
//! For example, `W1DMP2` refers to the second timeslot on a week one Monday,
//! `W2DRP5` refers to the last timeslot of a week two Thursday, and `W1DSP1`
//! is invalid (as no *active* day starts with that letter).
//!
//! In some cases, it may be necessary to refer to the iteration of a
//! timetable -- to refer to a specific iteration using this notation, prefix
//! the notation with `I#`, where `#` is the iteration (e.g., `I2W1DMP2` refers
//! to the second period on a Monday on the second iteration of week one).
//!
//! ## Activities
//!
//! An activity is a scheduled unit for performing a specific function -- for
//! Highfield, an activity can be any one of the following:
//!
//! * A lesson which a student/teacher is expected to attend.
//! * An assembly.
//! * Registration.
//! * A regular miscellaneous activity (e.g., driving lessons, therapy, etc).
//! * School study (where you study any subject in school).
//! * Home study (where you study any subject at home).
//! * A break (from work).
//!
//! Each activity must span one (or more) periods, and must not partially
//! utilise a periods (e.g., an activity may not use `1.5` periods), or
//! span several broken-up periods (e.g., an activity cannot span over
//! periods two and three as they are broken up by a twenty-minute break).
//!
//! [the Highfield school]: https://highfield.herts.sch.uk/
//! [the Fearnhill school]: https://fearnhill.herts.sch.uk/

pub use location::{FearnhillRoom, HighfieldBlock, HighfieldFloor, HighfieldRoom, Location};
pub use timeslot::{Period, TimeSlot, Week};
pub use ranged::*;

mod ranged;

/// This module contains data structures which describe the locations of rooms
/// within the Highfield school and the Fearnhill school.
mod location;

mod timeslot;
