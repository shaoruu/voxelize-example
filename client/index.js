import { Client } from '@voxelize/client';
import DirtImage from './assets/dirt.png';
import StoneImage from './assets/stone.png';

const domElement = document.getElementById('main');

const client = new Client({ container: { domElement } });

client.registry.applyTexturesByNames([
  { name: 'Dirt', side: 'all', data: DirtImage },
  { name: 'Stone', side: 'all', data: StoneImage },
]);

client.connect({ serverURL: 'http://localhost:4000' }).then(() => {
  client.join('Test');
});
