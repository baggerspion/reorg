import ApolloClient from 'apollo-boost';
import { ApolloProvider } from 'react-apollo';
import ConferenceList from '../components/conferences/ConferenceList';
import fetch from 'node-fetch';
import Layout from '../components/Layout';
import React, { Component } from 'react';
import { useParams } from 'react-router';

const { REACT_APP_FAUNADB_SECRET: secret } = process.env;

const client = new ApolloClient({
  uri: 'https://graphql.fauna.com/graphql',
  fetch,
  request: operation => {
    operation.setContext({
      headers: {
        authorization: `Bearer ${secret}`
      },
    });
  },
});

export default function Conferences({props}) {
  let { confID } = useParams();
  let conf = confID ? confID : null;
  
  if (conf) {
    return (
      <ApolloProvider client={client}>
        <Layout>
          <h1>{conf}</h1>
        </Layout>
      </ApolloProvider>
    );
  } else {
    return (
      <ApolloProvider client={client}>
        <Layout>
          <h1>Conferences</h1>
          <ConferenceList />
        </Layout>
      </ApolloProvider>
    );
  }
}