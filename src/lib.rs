use chrono::TimeZone;

const GET_VARS_URL: &str = "http://140.115.35.136:8044/cgi-bin/getVars.py";
const DOWNLOAD_URL: &str = "http://140.115.35.136:8044/cgi-bin/download.py";

fn _get_headers() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    headers.insert("charset", "utf-8".parse().unwrap());
    return headers;
}

pub async fn get_vars(
    start_time: &str,
    end_time: &str,
    user: &str,
) -> Result<String, reqwest::Error> {
    // parse start_time and end_time
    let start_time = chrono::Utc
        .datetime_from_str(start_time, "%Y-%m-%dT%H:%M")
        .unwrap()
        .format("%Y-%m-%dT%H:%M:%S")
        .to_string();
    let end_time = chrono::Utc
        .datetime_from_str(end_time, "%Y-%m-%dT%H:%M")
        .unwrap()
        .format("%Y-%m-%dT%H:%M:%S")
        .to_string();
    let time_range = &(start_time + "," + &end_time);

    // get base url
    let mut url = reqwest::Url::parse(GET_VARS_URL).unwrap();
    // append urlencoded string
    url.query_pairs_mut()
        .clear()
        .append_pair("timeRange", &time_range)
        .append_pair("user", user);

    // request
    let client = reqwest::Client::new();
    let text = client
        .post(url)
        .headers(_get_headers())
        .send()
        .await?
        .text()
        .await?;

    Ok(text)
}

pub async fn download(
    start_time: &str,
    end_time: &str,
    data_var: Vec<&str>,
) -> Result<String, reqwest::Error> {
    // parse start_time and end_time
    let start_time = chrono::Utc
        .datetime_from_str(start_time, "%Y-%m-%dT%H:%M")
        .unwrap()
        .format("%Y-%m-%dT%H:%M:%S")
        .to_string();
    let end_time = chrono::Utc
        .datetime_from_str(end_time, "%Y-%m-%dT%H:%M")
        .unwrap()
        .format("%Y-%m-%dT%H:%M:%S")
        .to_string();
    let time_period = &(start_time + "," + &end_time);

    // get base url
    let mut url = reqwest::Url::parse(DOWNLOAD_URL).unwrap();
    // append urlencoded string
    url.query_pairs_mut()
        .clear()
        .append_pair("dataVar", &data_var.join(","))
        .append_pair("timePeriod", time_period);

    // request
    let client = reqwest::Client::new();
    let text = client
        .post(url)
        .headers(_get_headers())
        .send()
        .await?
        .text()
        .await?;

    Ok(text)
}
