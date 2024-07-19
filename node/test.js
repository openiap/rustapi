const { Client, ClientError } = require('./main');
(async () => {
    try {
        const client = new Client('http://localhost:50051');
        console.log("now signing in");
        const signinResult = client.signin();
        console.log(signinResult);
        client.free();
    } catch (error) {
        if (error instanceof ClientError) {
            console.error(`An error occurred: ${error.message}`);
        } else {
            console.error(`An unexpected error occurred: ${error}`);
        }
    }
})();
