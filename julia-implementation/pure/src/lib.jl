using HTTP
using Comonicon

const GET_VARS_URL = "http://140.115.35.136:8044/cgi-bin/getVars.py"
const DOWNLOAD_URL = "http://140.115.35.136:8044/cgi-bin/download.py"
const HEADERS = ["Content-Type" => "application/x-www-form-urlencoded", "charset"=>"utf-8"]

@cast function get_vars(start_time, end_time; user="")
    response = HTTP.request(
        "POST",
        GET_VARS_URL,
        HEADERS;
        query=Dict(
            "timeRange" => start_time * "," * end_time,
            "user" => user
        )
    )
    println(String(response.body))
end

@cast function download(start_time, end_time, data_var...)
    response = HTTP.request(
        "POST",
        DOWNLOAD_URL,
        HEADERS;
        query=Dict(
            "timePeriod" => start_time * "," * end_time,
            "dataVar" =>  join(data_var, ",")
        )
    )
    println(String(response.body))
end
