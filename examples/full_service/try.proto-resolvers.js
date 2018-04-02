const grpc = require('grpc')
const Try = grpc.load('./try.proto')

const PizzeriaStub = new Try.Pizzeria(process.env.PIZZERIA_BACKEND_URL, grpc.credentials.createInsecure())

module.exports = {
  Query: {
    pizzeria: () => ({
      makeSimplePizza: ({ topping: req }) => {
        return new Promise((resolve, reject) => PizzeriaStub.MakeSimplePizza({...req}, (err, res) => err ? reject(err) : resolve(res)))
      },
    }),
  },
  Subscription: {
    pizzeria: () => ({
        observePizzas: (parent, { topping: req }, { pubsub }) => {
          const call = PizzeriaStub.ObservePizzas({...req})
          // taken from the graphql-yoga example
          // https://github.com/graphcool/graphql-yoga/blob/master/examples/subscriptions/index.jss
          const channel = Math.random().toString(36).substring(2, 15) // random channel name
          call.on('data', data => pubsub.publish(channel, data))
          call.on('end', () => true)
          call.on('status', () => true)
          return pubsub.asyncIterator(channel)
        },
    }),
  },
}