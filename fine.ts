import { App } from './index';

const main = async () => {
    const app = new App();
    await app.start();
}

main();