# SPL33N Moments Database

<iframe style="width:100%;height:500px" src="https://docs.google.com/spreadsheets/d/e/2PACX-1vRYf7Wp21r2gToSJ3PdhtkgfXQ9yo5iNAWY-FOxoreQJ4BQtTQlXCKYEkfTSBz9QL1x1IfKY87DvUsL/pubhtml?widget=true&amp;headers=false"></iframe>
[(view on google sheets)](https://docs.google.com/spreadsheets/d/1L9h2P-7q3v6gz4ldWr0C9s3ucL6pvpC3GJ94Qm13H9Y/edit?usp=sharing)

The **SPL33N Moments Database** is a set of indexed references to multiple humorous or otherwise memorable moments in the course of all of **CaptainSparklez** and **X33N**'s collaboration series. The database currently is and probably forever will be incomplete, and does not contain every single moment but hopes to provide an easy reference for episode searchers or a place of introduction for new viewers.

### Database entries follow a specific format:

- Three-character abbreviation of the series name
- Episode number. In case there is a conflict on this across perspectives, it will always be CS's episode number. The reason for this is that to date, CS has posted only a few solo episodes without X (sf3e15,17 + fkde1..2,6), whereas X has done more than six.
- Timecodes. Normally, only one timecode will be given if the event is most enjoyable from one perspective (the other cell is set NULL). If the event is enjoyable from both, both timecodes will be given. **Timecodes ARE NOT given in HH:MM:SS format**. Timecode format is raw seconds, which is the format used in YouTube's URL params. This makes it so that you can search YouTube for `[creator] [series] [ep. num]`, click on the relevant video, and append `&t=[timecode]` to the URL to jump to that specific time. You can use `time({col},'unixepoch')` in a raw SQLite query to convert `{col}` from `SSSS` to `HH:MM:SS` format.
- A brief description given in the present indicative and using **CS** and **X** to refer to **CaptainSparklez** and **X33N** for consistency and brevity.

### Spl command-line tool

The Database is embedded here as a google sheets file, but the live version I work on is an SQLite database which I manage from a custom program written in Rust. You can download `spl.db` from the repo (the link in the header) and put it into an external SQLite viewer or clone and compile the spl tool for yourself from the repo using `cargo build`. Put it in your `PATH` and run `spl -h` for help on what it does.

### Link generation

I used `youtube-dl` to fetch the video IDs for all of CS and X's collab playlists up to and not including Fallen Kingdom, and put them in the `urls.yml` file. The `spl` tool has the ability to generate timestamped `youtu.be` links for all of your fuzzy search results by using `spl fz -l`.

### Future projects

Soon, the Google Sheets embed above will contain auto-generated URLs instead of `series, episode, xtime, ctime`. It'll take me a while to figure out how to squash `spl.db` and `urls.yml` together into a `csv` file I can open in Sheets.
