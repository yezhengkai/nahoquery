using Downloads


dataVar = ["Rain01", "CWB_Humidity"]
start_time = "2022-03-10T00:00"
end_time = "2022-03-11T00:00"

dataVar = join(dataVar, "%2C")
timePeriod = replace(start_time, ":" => "%3A") * "%2C" * replace(end_time, ":" => "%3A")

url = "http://140.115.35.136:8044/cgi-bin/download.py?dataVar=$(dataVar)&timePeriod=$(timePeriod)"
headers = ["Content-Type" => "application/x-www-form-urlencoded", "charset"=>"utf-8"]

Downloads.download(
    url,
    stdout;
    method="POST",
    headers=headers,
)
# request(
#     url;
#     output=stdout,
#     method="POST",
#     headers=headers,
# )
