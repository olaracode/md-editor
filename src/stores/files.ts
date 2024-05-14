import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';

export type NotionFileT = {
	filename: string;
	content: string;
};

export async function getFiles() {
	let response = await invoke('get_files');
	return response as NotionFileT[];
}

export const files = writable<NotionFileT[]>([]);
