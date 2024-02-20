import { Octokit } from '@octokit/core';
import fs from 'fs';
import fetch from 'node-fetch';
import { release } from 'os';

const octokit = new Octokit({
	auth: process.env.GH_TOKEN
});

async function getReleaseForTag(tag) {
	let json = await octokit.request(
		'GET /repos/thulasi-ram/simple_android_debloater/releases/tags/{tag}',
		{
			tag: tag,
			headers: {
				'Content-Type': 'application/json',
				'X-GitHub-Api-Version': '2022-11-28'
			}
		}
	);

	let data = json.data;

	if (!data) {
		console.log(json);
		throw new Error('Error in response json');
	}

	return data;
}

async function downloadLatestJSON(release) {
	let latestJsonURL = '';

	for (let a of release.assets) {
		if (a.name == 'latest.json') {
			latestJsonURL = a.browser_download_url;
		}
	}

	if (!latestJsonURL) {
		throw new Error('latest.json is not present');
	}

	const response = await fetch(latestJsonURL);
	let json = await response.json();

	if (!json) {
		console.log(response);
		throw new Error('Error in response json');
	}

	return json;
}

async function getReleaseFromEventOrTag() {
	const releaseEvent = process.env.RELEASE_EVENT;
	const releaseTag = process.env.RELEASE_TAG;

	if (releaseEvent && releaseTag) {
		throw new Error('one of releaseTag or releaseEvent is expected. Got both');
	}

	if (!releaseEvent && !releaseTag) {
		throw new Error('one of releaseTag or releaseEvent is expected. Got neither');
	}

	console.log('release event and tag', releaseEvent, releaseTag);

	let release = {
		id: '',
		tag: '',
		body: '',
		assets: [],
		published_at: ''
	};

	if (releaseEvent) {
		let releaseData = JSON.parse(releaseEvent);
		console.log('release by event', releaseData.id);

		release = {
			id: releaseData.id,
			tag: releaseData.tag_name,
			body: releaseData.body,
			assets: releaseData.assets,
			published_at: releaseData.published_at
		};
	}

	if (releaseTag != '') {
		let releaseData = await getReleaseForTag(releaseTag);
		console.log('release by tag', releaseData.id);

		release = {
			id: releaseData.id,
			tag: releaseData.tag_name,
			body: releaseData.body,
			assets: releaseData.assets,
			published_at: releaseData.published_at
		};
	}

	let content = await downloadLatestJSON(release);
	content.notes = release.body;

	fs.writeFile('sad_updater.json', JSON.stringify(content), 'utf8', function (err) {
		if (err) throw err;
		console.log('Dumped sad_updater.json');
	});
}

getReleaseFromEventOrTag();
