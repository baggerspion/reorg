import gql from 'graphql-tag';

const submissionDetails = (id) => {
    return (
        gql`query submissionDetails{
            findSubmissionByID(id: "${id}") {
                abstract
                authors {
                    data {
                        first_name
                        last_name
                        email
                    }
                }
                status
                title
                reviews {
                    data {
                        score
                        public
                        private
                        reviewer {
                            reviewer {
                                first_name
                                last_name
                            }
                        }
                    }
                }
            }
        }`
    );
}

export default submissionDetails;