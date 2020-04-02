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

    let submissions = [];

    await client.paginate(
      q.Match(
        q.Index('subs_by_conference'), req.query.id
      )
    )    
    .map(ref => q.Get(ref))
    .each(page => {
      submissions = submissions.concat(page);
    });   
	  
    res.json(submissions);
  } catch (error) {
    res.status(500).json({ error });
  }     
}