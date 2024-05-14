import { invoke } from '@tauri-apps/api/tauri';
import type { NotionFileT } from '$lib/types';
async function getFiles() {
	let response = await invoke('get_files');
	return response as NotionFileT[];
}

export async function load() {
	return {
		files: await getFiles()
	};
}
