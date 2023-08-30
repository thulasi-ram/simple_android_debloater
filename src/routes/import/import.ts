import type { Config } from '$lib/config/models';
import { configStore } from '$lib/config/stores';
import { CSV_DIALOG_FILTER, JSON_DIALOG_FILTER } from '$lib/utils';
import { open, type DialogFilter } from '@tauri-apps/api/dialog';
import { readTextFile } from '@tauri-apps/api/fs';
import Papa, { type ParseResult } from 'papaparse';

export async function importSettingsJSON() {
	const openPath = await open({
		directory: false,
		multiple: false,
		title: 'Settings JSON',
		filters: [JSON_DIALOG_FILTER]
	});

	if (!openPath) return;

	if (Array.isArray(openPath)) {
		throw new Error('multiple selections for settings.json is not supported');
	}

	const fcontent = await readTextFile(openPath as string);

	const config: Config = JSON.parse(fcontent);
	configStore.set(config);
}

export async function openDialongSingleFile(
	title: string,
	filters: DialogFilter[]
): Promise<string | null> {
	const openPath = await open({
		directory: false,
		multiple: false,
		title: title,
		filters: filters
	});

	if (!openPath) return null;

	if (Array.isArray(openPath)) {
		throw new Error('multiple selections for settings.json is not supported');
	}

	return openPath as string;
}

export type PackageNames = string[];

type PackageRow = {
	package: string;
};

export async function openAndpParseCSVToJson(title: string) {
	let fpath: string = '';

	let openfpath = await openDialongSingleFile(title, [CSV_DIALOG_FILTER]);

	if (!openfpath) {
		return;
	}

	fpath = openfpath;

	return async () => {
		const fcontent = await readTextFile(fpath);
		let parseResult: ParseResult<PackageRow> = Papa.parse(fcontent, { header: true });
		if (parseResult.errors.length > 1) {
			throw new Error(`unable to parse csv ${parseResult.errors}`);
		}
		let packageNames: PackageNames = [];
		for (let p of parseResult.data) {
			packageNames.push(p['package']);
		}

		return packageNames;
	};
}
