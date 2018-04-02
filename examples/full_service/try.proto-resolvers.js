const grpc = require('grpc')
const Try = grpc.load('./try.proto')

const PizzeriaStub = new Try.Pizzeria(process.env.PIZZERIA_BACKEND_URL, grpc.credentials.createInsecure())

module.exports = {
  Query: {
    pizzeria: () => ({
      makeSimplePizza: ({ topping, req }) => {
        return new Promise((resolve, reject) => PizzeriaStub.MakeSimplePizza({...req}, (err, res) => err ? reject(err) : resolve(res)))
      },
    }),
  },
}