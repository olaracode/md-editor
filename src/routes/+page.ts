import { files, getFiles } from '../stores/files';

export async function load() {
	let filesFromSystem = await getFiles();
	files.set(filesFromSystem);
}
