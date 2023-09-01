import { Octokit } from '@octokit/core';
import fs from 'fs';

let template_json = {
	version: '0.5.0',
	notes: 'Test version',
	pub_date: '2023-06-22T19:25:57Z',
	platforms: {
		'darwin-x86_64': {
			signature: 'Content of app.tar.gz.sig',
			url: 'https://github.com/username/reponame/releases/download/v1.0.0/app-x86_64.app.tar.gz'
		},
		'darwin-aarch64': {
			signature: 'Content of app.tar.gz.sig',
			url: 'https://github.com/username/reponame/releases/download/v1.0.0/app-aarch64.app.tar.gz'
		},
		'linux-x86_64': {
			signature: 'Content of app.AppImage.tar.gz.sig',
			url: 'https://github.com/username/reponame/releases/download/v1.0.0/app-amd64.AppImage.tar.gz'
		},
		'windows-x86_64': {
			signature: 'Content of app-setup.nsis.sig or app.msi.sig, depending on the chosen format',
			url: 'https://github.com/username/reponame/releases/download/v1.0.0/app-x64-setup.nsis.zip'
		}
	}
};

async function getReleaseForTag(tag) {
	const octokit = new Octokit({
		auth: process.env.GH_TOKEN
	});

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

	let version = release.tag;
	if (version.charAt(0) === 'v') {
		version = version.substring(1); // remove first letter
	}

	template_json.version = version;
	template_json.notes = release.body;
	template_json.pub_date = release.published_at;

	fs.writeFile('sad_updater.json', JSON.stringify(template_json), 'utf8', function (err) {
		if (err) throw err;
		console.log('Dumped sad_updater.json');
	});
}

getReleaseFromEventOrTag();
