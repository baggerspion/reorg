import { ApolloClient } from 'apollo-client';
import { InMemoryCache } from 'apollo-cache-inmemory';
import { HttpLink } from 'apollo-link-http';
import fetch from 'isomorphic-unfetch';

const secret = process.env.FAUNADB_SECRET;

export default function createApolloClient(initialState, ctx) {
    // The `ctx` (NextPageContext) will only be present on the server.
    // use it to extract auth headers (ctx.req) or similar.
    return new ApolloClient({
        ssrMode: Boolean(ctx),
        link: new HttpLink({
            uri: 'https://graphql.fauna.com/graphql', // Server URL (must be absolute)
            headers: {
                authorization: `Bearer ${secret}`
            },
            fetch,
        }),
        cache: new InMemoryCache().restore(initialState),
    });
}