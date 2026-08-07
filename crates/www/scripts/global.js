/**
 * Fetch temperature data points.
 * 
 * @param {Number} [limit] - Number of data points to fetch, defaults to `10`.
 * @returns {Promise<Array<Object>>} A list of temperatures as objects.
 */
async function fetch_temperature(limit = 10) {
    try {
        const url = new URL(window.location.origin);
        url.pathname = "/api/temperature";
        url.searchParams.append("limit", limit);

        const response = await fetch(url);
        const out = response.json();

        return out;
    } catch (error) {
        throw error;
    }
}

/**
 * Fetch humidity data points.
 * 
 * @param {Number} [limit] - Number of data points to fetch, defaults to `10`.
 * @returns {Promise<Array<Object>>} A list of humidities as objects.
 */
async function fetch_humidity(limit = 10) {
    try {
        const url = new URL(window.location.origin);
        url.pathname = "/api/humidity";
        url.searchParams.append("limit", limit);

        const response = await fetch(url);
        const out = response.json();

        return out;
    } catch (error) {
        throw error;
    }
}

/**
 * Fetch position data points.
 * 
 * @param {Number} [limit] - Number of data points to fetch, defaults to `10`.
 * @returns {Promise<Array<Object>>} A list of positions as objects.
 */
async function fetch_position(limit = 10) {
    try {
        const url = new URL(window.location.origin);
        url.pathname = "/api/position";
        url.searchParams.append("limit", limit);

        const response = await fetch(url);
        const out = response.json();

        return out;
    } catch (error) {
        throw error;
    }
}
