import * as grpc from 'grpc'
import * as services from './services'

const PizzeriaStub = new services.Pizzeria(process.env.PIZZERIA_BACKEND_URL, grpc.credentials.createInsecure())

const resolvers = {
  Query: {
    pizzeria: {
      makeSimplePizza: async (req: any) => {
        const res = await PizzeriaStub.call(services.Pizzeria.MakeSimplePizza, req)
        return res.toJson()
      },
    },
  },
}