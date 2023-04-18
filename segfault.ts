import { App } from './index';

const main = async () => {
    let count = 0;
    const app = new App({
        payloadHandler: async (err, payload) => {
           count += 1;
           console.log(payload, count); 
        }
    });

    await app.start(100);
}

main();