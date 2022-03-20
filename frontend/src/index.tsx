/* @refresh reload */
import { render } from 'solid-js/web';

import './index.css';
import App from './components/App/App';
import Client from './util/client';
import makeNotifications from './components/util/notification/Notifications';

export const URL = "http://localhost:9000";
export const client = new Client(URL);

export const { Notifications, pushNotification } = makeNotifications();

render(() => [<App />, <Notifications />], document.getElementById('root') as HTMLElement);