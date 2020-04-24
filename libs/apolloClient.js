import { ApolloClient } from 'apollo-client';
import { InMemoryCache } from 'apollo-cache-inmemory';
import { HttpLink } from 'apollo-link-http';
import fetch from 'isomorphic-unfetch';

const secret = process.env.FAUNADB_SECRET;

export default function createApolloClient(initialState, ctx) {
    return new ApolloClient({
        ssrMode: Boolean(ctx),
        link: new HttpLink({
            uri: 'https://graphql.fauna.com/graphql',
            headers: {
                authorization: `Bearer ${secret}`
            },
            fetch,
        }),
        cache: new InMemoryCache().restore(initialState),
    });
}