import faunadb, { query as q } from 'faunadb';

const { FAUNADB_SECRET: secret } = process.env;

let client;

if (secret) {
  client = new faunadb.Client({ secret });
}

export default async (req, res) => {
  try {
    if (!client) {
      return [];
    }

    let conferences = [];

    await client.paginate(
      q.Match(
        q.Index('all_conferences')
      )
    )    
    .map(ref => q.Get(ref))
    .each(page => {
      conferences = conferences.concat(page);
    });
	  
    res.json({conferences});
  } catch (error) {
    res.status(500).json({ error });
  }
};
