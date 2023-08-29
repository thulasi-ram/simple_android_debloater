import { fetch } from '@tauri-apps/api/http';
import type { PackageDiscussion, PackageDiscussions } from './models';
import { packageDiscussionsStore } from './stores';

export async function getPackageDiscussions(): Promise<PackageDiscussions> {
	const response = await fetch<PackageDiscussion[]>(
		'https://d31d7prv3kbkn6.cloudfront.net/discussions_dump.json',
		{
			method: 'GET',
			timeout: 30
		}
	);
	let pdiscussions: PackageDiscussions = {};
	for (let pd of response.data) {
		pdiscussions[pd.title] = pd;
	}

	return pdiscussions;
}

export async function loadPackageDiscussions() {
    console.log("loading package discussions")
	const packageDiscussions = await getPackageDiscussions();
	packageDiscussionsStore.set(packageDiscussions);
    console.log(packageDiscussions);
}
