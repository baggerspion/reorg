import gql from 'graphql-tag';

const allConferenceSubmissionScores = (short_name) => {
    return (
        gql`query FindAllSubmissionScores {
            findConferenceByID(id: "${short_name}") {
                submissions {
                    data {
                        _id
                        title
                        status
                        reviews {
                            data {
                                score
                            }
                        }
                    }
                }
            }
        }`
    );
}

export default allConferenceSubmissionScores;