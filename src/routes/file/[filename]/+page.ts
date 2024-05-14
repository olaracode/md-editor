import { files, getFiles } from '../../../stores/files';
import { get } from 'svelte/store';
import { page } from '$app/stores';

export async function load({ params }) {
	let filesValue = get(files);
	if (!filesValue || filesValue.length === 0) {
		filesValue = await getFiles();
		files.set(filesValue);
	}
	const file = filesValue.find((file) => file.filename === params.filename);
	if (!file) {
		return { status: 404, error: new Error('File not found') };
	}

	return { file };
}
