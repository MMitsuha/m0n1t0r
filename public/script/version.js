async function onload() {
    await version();
    await clients();
}

async function version() {
    const body = await get('./server');

    document.getElementById('version').innerText = body.version;
    document.getElementById('build_time').innerText = body.build_time;
}

async function clients() {
    const body = await get('./client');
    
    // TODO: Implement this
    console.log(body);
}
