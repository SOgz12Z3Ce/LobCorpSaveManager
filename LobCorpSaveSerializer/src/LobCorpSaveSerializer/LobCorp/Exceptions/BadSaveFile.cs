using System;

namespace LobCorp.Exceptions
{
	public class BadSaveFile : Exception
	{
		public BadSaveFile()
		{
		}
		public BadSaveFile(string message) : base(message)
		{
		}
		public BadSaveFile(string message, Exception innerException) : base(message, innerException)
		{
		}
	}
}
