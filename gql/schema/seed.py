from dotenv import load_dotenv

from faker import Faker

from faunadb import query as q
from faunadb.objects import Ref
from faunadb.client import FaunaClient

import random, os

load_dotenv()
client = FaunaClient(os.environ['REACT_APP_FAUNADB_SECRET'])
fake = Faker(['it_IT', 'en_GB', 'de_DE', 'cs_CZ'])

def create_users():
    users = []

    for _ in range(1000):
        new_user = {
            "first_name": fake.first_name(),
            "last_name": fake.last_name(),
            "email": fake.ascii_free_email()
        }
        users.append(new_user)

    return users

def create_conferences():
    conferences = []

    for _ in range(20):
        new_conference = {
            "name": fake.bs(),
            "short_name": fake.slug(),
            "start_date": q.date(fake.date()),
            "end_date": q.date(fake.date()),
            "venue": fake.company(),
            "address": fake.address(),
            "country": fake.country(),
            "email": fake.ascii_free_email(),
            "web": fake.uri(),
        }
        conferences.append(new_conference)

    return conferences

def create_reviewers():
    reviewers = []
    users = client.query(q.paginate(q.match(q.index("allUsers")), size=1000))
    conferences = client.query(q.paginate(q.match(q.index("allConferences")), size=1000))

    for conf in conferences['data']:
        for user in random.sample(users['data'], 7):
            new_committee_member = {
                "conferenceID": conf,
                "userID": user
            }
            reviewers.append(new_committee_member)

    return reviewers

def create_submissions():
    submissions = []
    conferences = client.query(q.paginate(q.match(q.index("allConferences")), size=1000))

    for conference in conferences['data']:
        for _ in range(random.randint(1, 30)):
            new_submission = {
                "conference": conference,
                "accepted": False,
                "title": fake.sentence(nb_words=random.randint(3, 6) + 1),
                "abstract": fake.paragraph(),
            }
            submissions.append(new_submission)

    return submissions

def create_authors():
    authors = []
    users = client.query(q.paginate(q.match(q.index("allUsers")), size=1000))
    submissions = client.query(q.paginate(q.match(q.index("allSubmissions")), size=1000))

    for paper in submissions['data']:
        for author in random.sample(users['data'], random.randint(0, 1) + 1):
            new_author = {
                "submissionID": paper,
                "userID": author
            }
            authors.append(new_author)

    return authors

def insert_data(collection, data):
    client.query(
        q.map_(
            lambda item: q.create(
                q.collection(collection),
                {"data": item}
            ),
            data
        )
    )

if __name__ == '__main__':
    insert_data("User", create_users())
    insert_data("Conference", create_conferences())
    insert_data("Reviewer", create_reviewers())
    insert_data("Submission", create_submissions())
    insert_data("Author", create_authors())