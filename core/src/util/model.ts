import { Collection } from "@monitor/types";
import { objFrom2Arrays } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { Schema, FilterQuery, QueryOptions } from "mongoose";

// a custom api on top of mongoose models with assumptive types / custom convenience queries
const model = <T>(app: FastifyInstance, name: string, schema: Schema<T>) => {
  const model = app.mongoose.model<T>(name, schema);

  return {
    create: async (item: T) => {
      return (await model.create(item)) as T;
    },
    find: async (
      filter: FilterQuery<T> = {},
      projection?: string | object,
      options?: QueryOptions
    ) => {
      return (await model.find(filter, projection, options)) as T[];
    },
    findById: async (
      id: string,
      projection?: string | object,
      options?: QueryOptions
    ) => {
      return (await model.findById(id, projection, options)) as T | undefined;
    },
    findByField: async <Target>(
      field: string,
      expr: Target,
      projection?: string | object,
      options?: QueryOptions
    ) => {
      return (await model.find(
        { [field]: expr } as FilterQuery<T>,
        projection,
        options
      )) as T[];
    },
    findOne: async (
      filter: FilterQuery<T> = {},
      projection?: string | object,
      options?: QueryOptions
    ) => {
      return (await model.findOne(filter, projection, options)) as
        | T
        | undefined;
    },
    getMostRecent: async (
      limit: number,
      filter: FilterQuery<T>,
      projection?: string | object,
      options?: QueryOptions
    ) => {
      return (await model
        .find(filter, projection, options)
        .sort({ createdAt: -1 })
        .limit(limit)) as T[];
    },
    findCollection: async (
      filter: FilterQuery<T>,
      projection?: string | object,
      options?: QueryOptions
    ) => {
      const docs = await model.find(filter, projection, options);
      return objFrom2Arrays(
        docs.map((doc) => doc._id),
        docs
      ) as Collection<T>;
    },
  };
};

// this is just used to derive the returntype of a the generic model
class CreateModel<T> {
  mediate = (app: FastifyInstance, name: string, schema: Schema<T>) =>
    model<T>(app, name, schema);
}

export type Model<T> = ReturnType<CreateModel<T>["mediate"]>;

export default model;
