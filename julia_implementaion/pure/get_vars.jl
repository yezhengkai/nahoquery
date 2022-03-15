using Downloads


start_time = "2022-03-10T00:00"
end_time = "2022-03-11T00:00"

timeRange = replace(start_time, ":" => "%3A") * "%2C" * replace(end_time, ":" => "%3A")
user = ""

url = "http://140.115.35.136:8044/cgi-bin/getVars.py?timeRange=$(timeRange)&user=$(user)"
headers = ["Content-Type" => "application/x-www-form-urlencoded", "charset"=>"utf-8"]


request(
    url;
    output=stdout,
    method="POST",
    headers=headers,
)
