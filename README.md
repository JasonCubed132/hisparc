# A library / CLI intended for downloading and processing HiSPARC data
This project is in its *very* early stages and should not be used

# API notes

## All other data

Base URL: `https://data.hisparc.nl/api/`
- Responds with all sub-URLs, with the exception of how to actually retrieve data.

## Event/weather data
Events URL: `https://data.hisparc.nl/data/{station_number}/events`
Weather URL: `https://data.hisparc.nl/data/{station_number}/weather`

Both of these must have query parameters added to them:
- download: bool - Indicates whether to stream or not (CHECK THIS)
- start: datetime - Indicates the start of the downloaded period
- end: datatime - Indicates the end of the downloaded period

Datetime outputs: `YYYY-MM-DD HH-MM-SS`

Download page is here: `https://data.hisparc.nl/data/download/`
- Source appears to send a get request based on the contents of the form but Firefox's network debugger doesn't show anything.