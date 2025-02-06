async function get(url) {
    const response = await fetch(url);
    const json = await response.json();

    if (json.code != 0) {
        const msg = 'failed: ' + json.body;
        alert(msg);
        throw new Error(msg);
    }

    return json.body;
}
