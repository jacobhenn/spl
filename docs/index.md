# SPL33N Moments Database

<iframe style="width:100%;height:500px" src="https://docs.google.com/spreadsheets/d/e/2PACX-1vRYg70qFGm5OsyAgbzOdTR1E_I1nFYWDVj8hjR0fQYqYofyxuZSBHXCjFnxjvRGGfWQ1GCp4ReVFhCN/pubhtml?gid=1549579440&amp;single=true&amp;widget=true&amp;headers=false"></iframe>
[view on google sheets](https://docs.google.com/spreadsheets/d/1mdxf4r6X5gEuClQ_vE1orCUxeqgveJLobaCYlApt4vw/edit?usp=sharing)

[download as `.csv`](https://raw.githubusercontent.com/jacobhenn/spl/main/resources/moments.csv)

[download as SQLite `.db` (w/o links, see **Link Generaton** below)](https://raw.githubusercontent.com/jacobhenn/spl/main/resources/spl.db)

The **SPL33N Moments Database** is a set of indexed references to multiple humorous or otherwise memorable moments in the course of all of **CaptainSparklez** and **X33N**'s collaboration series. The database currently is and probably forever will be incomplete, and does not contain every single moment but hopes to provide an easy reference for episode searchers or a place of introduction for new viewers.

## Spl command-line tool

The Database is embedded here as a google sheets file, but the live version I work on is an SQLite database which I manage from a custom program written in Rust. You can download `spl.db` from the github repo and put it into an external SQLite viewer, or clone and compile the spl tool for yourself from the repo by installing [the rust toolchain](https://rust-lang.org/install) and running `cargo install --path .` in the project directory. The files `spl.db` and `urls.yml` should always remain in the same place they were at compile time, but you can delete other files or run `cargo clean` after compiling so the leftovers don't take up too much disc space. Run `spl -h` for help on what it does (if it can't find the executable, make sure the cargo root installation directory is in your `PATH`).

### SQLite database entries follow a specific format:

- row "series": Three-character abbreviation of the series name
- row "episode": Episode number. In case there is a conflict on this across perspectives, it will always be CS's episode number. The reason for this is that to date, CS has posted only a few solo episodes without X (sf3e15,17 + fkde1..2,6), whereas X has done more than six.
- rows "xtime" and "ctime": Timecodes. Normally, only one timecode will be given if the event is most enjoyable from one perspective (the other cell is set NULL). If the event is enjoyable from both, both timecodes will be given. **Timecodes ARE NOT given in HH:MM:SS format**. Timecode format is raw seconds, which is the format used in YouTube's URL params. This makes it so that you can search YouTube for `[creator] [series] [ep. num]`, click on the relevant video, and append `&t=[timecode]` to the URL to jump to that specific time. You can use `time({col},'unixepoch')` in a raw SQLite query to convert `{col}` from `SSSS` to `HH:MM:SS` format.
- row "desc": A brief description given in the present indicative and using **CS** and **X** to refer to **CaptainSparklez** and **X33N** for consistency and brevity.
- **Do not use double quotes ⟨"⟩ within the description and do not use commas ⟨,⟩ within the series or episode**, as this will probably do bad things to the auto-generated CSV file.

### Link generation

I used `youtube-dl` to fetch the video IDs for all of CS and X's collab playlists up to and not including Fallen Kingdom, and put them in the `urls.yml` file. The `spl` tool has the ability to generate timestamped `youtu.be` links for all of your fuzzy search results by using `spl fz -l`. You can also use `spl gencsv` to generate a file named `moments.csv` (in the directory in which you ran the command) containing rows for X's url, CS's url, and the description.
  
  
  
<p xmlns:dct="http://purl.org/dc/terms/">
  <a rel="license"
     href="http://creativecommons.org/publicdomain/zero/1.0/">
    <img src="https://licensebuttons.net/p/zero/1.0/88x31.png" style="border-style: none;" alt="CC0" />
  </a>
  <br />
  To the extent possible under law,
  <a rel="dct:publisher"
     href="https://github.com/jacobhenn">
    <span property="dct:title">Jacob Henn</span></a>
  has waived all copyright and related or neighboring rights to
  <span property="dct:title">the SPL33N Moments Database</span>.
</p>
