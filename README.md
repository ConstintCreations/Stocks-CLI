#Stocks CLI
This is a simple CLI made in Rust to view the top 10 stock gainers and losers and look up a ticker's price and daily change.

##Usage
To use, clone this repository and add a .env file. Within the file, type: <p>ALPHAVANTAGEAPIKEY=</p> followed by an api key obtained from [AlphaVantage](https://www.alphavantage.co/support/#api-key).

Then, you can type "run cargo -- -h" in your terminal, to see a how to use the CLI. It will output the following:

<br>

Usage: stocks_cli.exe [OPTIONS] [ticker]

Arguments: <br>
  [ticker]
  
Options:  <br>
  -t, --top  <br>
  -h, --help     Print help  <br>
  -V, --version  Print version
