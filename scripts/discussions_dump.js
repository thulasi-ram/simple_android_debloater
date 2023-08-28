// https://stepzen.com/blog/consume-graphql-in-javascript
// https://stackoverflow.com/questions/42938472/what-is-the-reason-for-having-edges-and-nodes-in-a-connection-in-your-graphql-sc
// https://docs.github.com/en/graphql/guides/using-the-graphql-api-for-discussions
import fetch from 'node-fetch';
import fs from 'fs';

async function getDiscussions(no_of_discussions, next_cursor) {
	const reqData = JSON.stringify({
		query: `query($no_of_discussions:Int!, $next_cursor: String) {
        repository(owner: "thulasi-ram", name: "simple_android_debloater") {
          discussions(first: $no_of_discussions, after: $next_cursor) {
            # type: DiscussionConnection
            totalCount # Int!
      
            pageInfo {
              startCursor
              endCursor
              hasNextPage
              hasPreviousPage
            }

            nodes {
              # type: Discussion
              id
              title
              closed
              body
              bodyHTML
              answer {
                body
                bodyHTML
              }
              labels(first: 100) {
                totalCount
                nodes {
                  name
                  description
                }
              }

            }
          }
        }
      }`,
		variables: `{
        "no_of_discussions": ${no_of_discussions},
        "next_cursor": ${next_cursor == null ? null : `"${next_cursor}"`}
      }`
	});

	const response = await fetch('https://api.github.com/graphql', {
		method: 'post',
		body: reqData,
		headers: {
			'Content-Type': 'application/json',
			'Content-Length': reqData.length,
			Authorization: 'bearer ' + process.env.GH_TOKEN,
			'User-Agent': 'Node'
		}
	});
	const json = await response.json();

	let data = json.data;

	if (!data) {
		console.log(response);
		console.log(json);
		throw new Error('Error in response json');
	}

	const discussions = data?.repository?.discussions;

	if (!discussions) {
		console.log(JSON.stringify(data));
	}

	return discussions;
}

async function getAllDiscussions() {
	// https://stackoverflow.com/questions/71952373/how-to-run-graphql-query-in-a-loop-until-condition-is-not-matched

	let hasNext = true;
	let nextCursor = null;
	let allDiscussions = [];
	while (hasNext) {
		let discussionsData = await getDiscussions(1, nextCursor);
		if (!discussionsData) {
			break;
		}
		hasNext = discussionsData.pageInfo.hasNextPage;
		nextCursor = discussionsData.pageInfo.endCursor;
		allDiscussions.push(...discussionsData.nodes);
	}

	allDiscussions = allDiscussions.map((d) => parseDiscussion(d)).filter((d) => d != null);
	let discussionsDump = JSON.stringify(allDiscussions);
	console.log(discussionsDump);

	fs.writeFile('discussions_dump.json', discussionsDump, 'utf8', function (err) {
		if (err) throw err;
		console.log('Dumped json');
	});
}

const FILTER_BY_PACKAGE_LABEL = 'pkg';

function parseDiscussion(discussion) {
	if (!discussion) {
		return null;
	}

	const labelCount = discussion.labels?.totalCount;
	if (labelCount && labelCount > 100) {
		console.error(`"discussoon ${discussion} has more than 100 labels"`);
	}
	const labels = discussion.labels?.nodes;
	if (!labels) {
		return null;
	}

	let matched = labels.filter((l) => l.name === FILTER_BY_PACKAGE_LABEL);
	if (!matched) {
		return null;
	}
	discussion.labels = labels;
	return discussion;
}

getAllDiscussions();
