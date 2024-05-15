import { files, getFiles } from '../../../stores/files';
import { get } from 'svelte/store';

export async function load({ params }) {
	let filesValue = get(files);
	if (!filesValue || filesValue.length === 0) {
		filesValue = await getFiles();
		files.set(filesValue);
	}
	return { params };
}
