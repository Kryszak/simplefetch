# simplefetch

[![Codacy Badge](https://api.codacy.com/project/badge/Grade/0d8ca215a3374d4383452d0f6a7efcc3)](https://app.codacy.com/gh/Kryszak/simplefetch?utm_source=github.com&utm_medium=referral&utm_content=Kryszak/simplefetch&utm_campaign=Badge_Grade_Settings)

Simple fetch tool for Linux OS

## Usage
```
$ ./simplefetch -h
Usage:
  ./simplefetch [OPTIONS]

Simple Linux system information fetcher.

Optional arguments:
  -h,--help             Show this help message and exit
  --label-padding LABEL_PADDING
                        Padding of section labels
  --style STYLE         Style of display. Available: left,right
```

## Options
```
--label-padding LABEL_PADDING # sets number of blank space added before or after label
--style STYLE # sets display style. "left" -> Labels are justified to the left side and padding is added after them,
"right" -> Labels are justified to the right side and padding is added before them,
"icon" -> use icons instead of text labels
```
