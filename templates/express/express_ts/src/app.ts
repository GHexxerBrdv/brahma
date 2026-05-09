import express, { Request, Response, NextFunction } from 'express';
import cors from 'cors';
import morgan from 'morgan';

// import userRoutes from './routes/user.routes';
// import errorMiddleware from './middlewares/error.middleware';

const app = express();

app.use(cors());
app.use(morgan('dev'));
app.use(express.json());

app.get('/', (req: Request, res: Response) => {
  res.json({
    success: true,
    message: 'API Running (TypeScript)',
  });
});

// app.use('/api/users', userRoutes);

// app.use(errorMiddleware);

export default app;
