const express = require('express');
const cors = require('cors');
const morgan = require('morgan');

const userRoutes = require('./routes/user.routes');
const errorMiddleware = require('./middlewares/error.middleware');

const app = express();

app.use(cors());
app.use(morgan('dev'));
app.use(express.json());

app.get('/', (req, res) => {
  res.json({
    success: true,
    message: 'API Running',
  });
});

app.use('/api/users', userRoutes);

app.use(errorMiddleware);

module.exports = app;